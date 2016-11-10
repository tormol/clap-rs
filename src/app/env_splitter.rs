#[cfg(windows)]
use ::osstringext::OsStrExt3;
#[cfg(not(windows))]
use std::os::unix::ffi::OsStrExt;

pub struct SplitArgs {
    args: Vec<u8>,
    pos: usize,
}
impl SplitArgs {
    pub fn new(args: OsString) -> Self {
        SplitArgs{
            args: args.as_bytes().to_vec(),
            pos: 0,
        }
    }
}
impl Iterator for SplitArgs {
    type Item = OsString;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += self.args[self.pos..]
                        .iter()
                        .take_while(|&b| b==b' ' || b==b'\t' || b==b'\n' )
                        .count();
        if self.pos == self.args.len() {
            return None;
        }

        enum State {Normal,SingleQ,DoubleQ,Escape,DQEscape}
        use ::State:*;
        let mut state = Normal;
        let mut arg = Vec::new();
        loop {
            let b = match self.args.get(self.pos) {
                Some(b) => *b,
                None => {// no more bytes
                    match state {// flush state
                        Escape    =>  arg.push(b'\\'),
                        DQEscape  =>  arg.push(b'\\'),
                        _         =>  ()
                    }
                    break;
                }
            }
            self.pos += 1;
            state = match (state, b) {
                (Normal,   b' ')   =>  break,
                (Normal,   b'\t')  =>  break,
                (Normal,   b'\n')  =>  break,
                (Normal,   b'\'')  =>                                 SingleQ ,
                (Normal,   b'"')   =>                                 DoubleQ ,
                (Normal,   b'\\')  =>                                 NEscape ,
                (Normal,   _)      =>  {                 arg.push(b); Normal },
                (NEscape,  _)      =>  {                 arg.push(b); Normal },
                (SingleQ,  b'\'')  =>                                 Normal  ,
                (SingleQ,  _)      =>  {                 arg.push(b); SingleQ},
                (DoubleQ,  b'"')   =>                                 Normal  ,
                (DoubleQ,  b'\\')  =>                                 DQEscape,
                (DoubleQ,  _)      =>  {                 arg.push(b); DoubleQ},
                (DQEscape, b'\n')  =>  {                 arg.push(b); DQuote },
                (DQEscape, b'\\')  =>  {                 arg.push(b); DQuote },
                (DQEscape, b'"')   =>  {                 arg.push(b); DQuote },
                (DQEscape, _)      =>  {arg.push(b'\\'); arg.push(b); DQuote },
            };
        }
        Some(OsString::from(unsafe{ String::from_utf8_unchecked(arg) }))
    }
}

