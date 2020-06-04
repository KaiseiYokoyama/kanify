# Kanify

ひらがな / カタカナ間, 小文字/大文字間, および清音/濁音/半濁音間の判定, 変換を補助するクレートです. 

```rust
use kanify::*;

assert_eq!('あ'.katakanify(), Ok('ア'));
assert_eq!('ア'.hiraganify(), Ok('あ'));
assert_eq!('あ'.komojify(), Ok('ぁ'));
assert_eq!('ぁ'.oomojify(), Ok('あ'));
assert_eq!('は'.dakuonify(), Ok('ば'));
assert_eq!('ば'.handakuonify(), Ok('ぱ'));
assert_eq!('ぱ'.seionify(), Ok('は'));
```