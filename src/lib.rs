#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doctest!("../README.md");

#[cfg(test)]
mod tests {
    use crate::Kanify;

    #[test]
    fn test() {
        assert_eq!('あ'.komojify(), Ok('ぁ'));
        assert_eq!('は'.dakuonify(), Ok('ば'));
        assert_eq!('ば'.handakuonify(), Ok('ぱ'));
        assert_eq!('ぱ'.seionify(), Ok('は'));
    }

    #[test]
    fn is_hiragana() {
        assert!(
            [
                'ぁ', 'あ', 'ぃ', 'い', 'ぅ', 'う', 'ぇ', 'え', 'ぉ', 'お', 'か', 'が', 'き', 'ぎ', 'く',
                'ぐ', 'け', 'げ', 'こ', 'ご', 'さ', 'ざ', 'し', 'じ', 'す', 'ず', 'せ', 'ぜ', 'そ', 'ぞ', 'た',
                'だ', 'ち', 'ぢ', 'っ', 'つ', 'づ', 'て', 'で', 'と', 'ど', 'な', 'に', 'ぬ', 'ね', 'の', 'は',
                'ば', 'ぱ', 'ひ', 'び', 'ぴ', 'ふ', 'ぶ', 'ぷ', 'へ', 'べ', 'ぺ', 'ほ', 'ぼ', 'ぽ', 'ま', 'み',
                'む', 'め', 'も', 'ゃ', 'や', 'ゅ', 'ゆ', 'ょ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'ゎ', 'わ',
                'ゐ', 'ゑ', 'を', 'ん', 'ゔ',
            ].iter()
                .all(|c| c.clone().is_hiragana())
        );

        assert!(
            [
                'ァ', 'ア', 'ィ', 'イ', 'ゥ', 'ウ', 'ェ', 'エ', 'ォ', 'オ', 'カ', 'ガ', 'キ', 'ギ', 'ク',
                'グ', 'ケ', 'ゲ', 'コ', 'ゴ', 'サ', 'ザ', 'シ', 'ジ', 'ス', 'ズ', 'セ', 'ゼ', 'ソ', 'ゾ', 'タ',
                'ダ', 'チ', 'ヂ', 'ッ', 'ツ', 'ヅ', 'テ', 'デ', 'ト', 'ド', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ',
                'バ', 'パ', 'ヒ', 'ビ', 'ピ', 'フ', 'ブ', 'プ', 'ヘ', 'ベ', 'ペ', 'ホ', 'ボ', 'ポ', 'マ', 'ミ',
                'ム', 'メ', 'モ', 'ャ', 'ヤ', 'ュ', 'ユ', 'ョ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ヮ', 'ワ',
                'ヰ', 'ヱ', 'ヲ', 'ン', 'ヴ',
            ].iter()
                .all(|c| !c.clone().is_hiragana())
        );
    }

    #[test]
    fn hiraganify() {
        assert_eq!(
            [
                'ァ', 'ア', 'ィ', 'イ', 'ゥ', 'ウ', 'ェ', 'エ', 'ォ', 'オ', 'カ', 'ガ', 'キ', 'ギ', 'ク',
                'グ', 'ケ', 'ゲ', 'コ', 'ゴ', 'サ', 'ザ', 'シ', 'ジ', 'ス', 'ズ', 'セ', 'ゼ', 'ソ', 'ゾ', 'タ',
                'ダ', 'チ', 'ヂ', 'ッ', 'ツ', 'ヅ', 'テ', 'デ', 'ト', 'ド', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ',
                'バ', 'パ', 'ヒ', 'ビ', 'ピ', 'フ', 'ブ', 'プ', 'ヘ', 'ベ', 'ペ', 'ホ', 'ボ', 'ポ', 'マ', 'ミ',
                'ム', 'メ', 'モ', 'ャ', 'ヤ', 'ュ', 'ユ', 'ョ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ヮ', 'ワ',
                'ヰ', 'ヱ', 'ヲ', 'ン', 'ヴ',
            ].iter()
                .flat_map(|c| c.clone().hiraganify())
                .collect::<Vec<char>>(),
            vec![
                'ぁ', 'あ', 'ぃ', 'い', 'ぅ', 'う', 'ぇ', 'え', 'ぉ', 'お', 'か', 'が', 'き', 'ぎ', 'く',
                'ぐ', 'け', 'げ', 'こ', 'ご', 'さ', 'ざ', 'し', 'じ', 'す', 'ず', 'せ', 'ぜ', 'そ', 'ぞ', 'た',
                'だ', 'ち', 'ぢ', 'っ', 'つ', 'づ', 'て', 'で', 'と', 'ど', 'な', 'に', 'ぬ', 'ね', 'の', 'は',
                'ば', 'ぱ', 'ひ', 'び', 'ぴ', 'ふ', 'ぶ', 'ぷ', 'へ', 'べ', 'ぺ', 'ほ', 'ぼ', 'ぽ', 'ま', 'み',
                'む', 'め', 'も', 'ゃ', 'や', 'ゅ', 'ゆ', 'ょ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'ゎ', 'わ',
                'ゐ', 'ゑ', 'を', 'ん', 'ゔ',
            ]
        )
    }

    #[test]
    fn is_katakana() {
        assert!(
            [
                'ぁ', 'あ', 'ぃ', 'い', 'ぅ', 'う', 'ぇ', 'え', 'ぉ', 'お', 'か', 'が', 'き', 'ぎ', 'く',
                'ぐ', 'け', 'げ', 'こ', 'ご', 'さ', 'ざ', 'し', 'じ', 'す', 'ず', 'せ', 'ぜ', 'そ', 'ぞ', 'た',
                'だ', 'ち', 'ぢ', 'っ', 'つ', 'づ', 'て', 'で', 'と', 'ど', 'な', 'に', 'ぬ', 'ね', 'の', 'は',
                'ば', 'ぱ', 'ひ', 'び', 'ぴ', 'ふ', 'ぶ', 'ぷ', 'へ', 'べ', 'ぺ', 'ほ', 'ぼ', 'ぽ', 'ま', 'み',
                'む', 'め', 'も', 'ゃ', 'や', 'ゅ', 'ゆ', 'ょ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'ゎ', 'わ',
                'ゐ', 'ゑ', 'を', 'ん', 'ゔ',
            ].iter()
                .all(|c| !c.clone().is_katakana())
        );

        assert!(
            [
                'ァ', 'ア', 'ィ', 'イ', 'ゥ', 'ウ', 'ェ', 'エ', 'ォ', 'オ', 'カ', 'ガ', 'キ', 'ギ', 'ク',
                'グ', 'ケ', 'ゲ', 'コ', 'ゴ', 'サ', 'ザ', 'シ', 'ジ', 'ス', 'ズ', 'セ', 'ゼ', 'ソ', 'ゾ', 'タ',
                'ダ', 'チ', 'ヂ', 'ッ', 'ツ', 'ヅ', 'テ', 'デ', 'ト', 'ド', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ',
                'バ', 'パ', 'ヒ', 'ビ', 'ピ', 'フ', 'ブ', 'プ', 'ヘ', 'ベ', 'ペ', 'ホ', 'ボ', 'ポ', 'マ', 'ミ',
                'ム', 'メ', 'モ', 'ャ', 'ヤ', 'ュ', 'ユ', 'ョ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ヮ', 'ワ',
                'ヰ', 'ヱ', 'ヲ', 'ン', 'ヴ',
            ].iter()
                .all(|c| c.clone().is_katakana())
        );
    }

    #[test]
    fn katakanify() {
        assert_eq!(
            [
                'ぁ', 'あ', 'ぃ', 'い', 'ぅ', 'う', 'ぇ', 'え', 'ぉ', 'お', 'か', 'が', 'き', 'ぎ', 'く',
                'ぐ', 'け', 'げ', 'こ', 'ご', 'さ', 'ざ', 'し', 'じ', 'す', 'ず', 'せ', 'ぜ', 'そ', 'ぞ', 'た',
                'だ', 'ち', 'ぢ', 'っ', 'つ', 'づ', 'て', 'で', 'と', 'ど', 'な', 'に', 'ぬ', 'ね', 'の', 'は',
                'ば', 'ぱ', 'ひ', 'び', 'ぴ', 'ふ', 'ぶ', 'ぷ', 'へ', 'べ', 'ぺ', 'ほ', 'ぼ', 'ぽ', 'ま', 'み',
                'む', 'め', 'も', 'ゃ', 'や', 'ゅ', 'ゆ', 'ょ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'ゎ', 'わ',
                'ゐ', 'ゑ', 'を', 'ん', 'ゔ',
            ].iter()
                .flat_map(|c| c.clone().katakanify())
                .collect::<Vec<char>>(),
            vec![
                'ァ', 'ア', 'ィ', 'イ', 'ゥ', 'ウ', 'ェ', 'エ', 'ォ', 'オ', 'カ', 'ガ', 'キ', 'ギ', 'ク',
                'グ', 'ケ', 'ゲ', 'コ', 'ゴ', 'サ', 'ザ', 'シ', 'ジ', 'ス', 'ズ', 'セ', 'ゼ', 'ソ', 'ゾ', 'タ',
                'ダ', 'チ', 'ヂ', 'ッ', 'ツ', 'ヅ', 'テ', 'デ', 'ト', 'ド', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ',
                'バ', 'パ', 'ヒ', 'ビ', 'ピ', 'フ', 'ブ', 'プ', 'ヘ', 'ベ', 'ペ', 'ホ', 'ボ', 'ポ', 'マ', 'ミ',
                'ム', 'メ', 'モ', 'ャ', 'ヤ', 'ュ', 'ユ', 'ョ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ヮ', 'ワ',
                'ヰ', 'ヱ', 'ヲ', 'ン', 'ヴ',
            ]
        )
    }

    #[test]
    fn is_komoji() {
        assert!([
            'あ', 'い', 'う', 'え', 'お', 'つ', 'や', 'ゆ', 'よ', 'わ',
            'ア', 'イ', 'ウ', 'エ', 'オ', 'ツ', 'ヤ', 'ユ', 'ヨ', 'ワ',
        ].iter()
            .all(|c| !c.clone().is_komoji())
        );
        assert!([
            'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ',
            'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ',
        ].iter()
            .all(|c| c.clone().is_komoji())
        );
    }

    #[test]
    fn komojify() {
        assert_eq!(
            [
                'あ', 'い', 'う', 'え', 'お', 'つ', 'や', 'ゆ', 'よ', 'わ',
                'ア', 'イ', 'ウ', 'エ', 'オ', 'ツ', 'ヤ', 'ユ', 'ヨ', 'ワ',
            ].iter()
                .flat_map(|c| c.komojify())
                .collect::<Vec<char>>(),
            vec![
                'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ',
                'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ',
            ]
        );
    }

    #[test]
    fn is_oomoji() {
        assert!([
            'あ', 'い', 'う', 'え', 'お', 'つ', 'や', 'ゆ', 'よ', 'わ',
            'ア', 'イ', 'ウ', 'エ', 'オ', 'ツ', 'ヤ', 'ユ', 'ヨ', 'ワ',
        ].iter()
            .all(|c| c.clone().is_oomoji())
        );
        assert!([
            'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ',
            'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ',
        ].iter()
            .all(|c| !c.clone().is_oomoji())
        );
    }

    #[test]
    fn oomojify() {
        assert_eq!(
            [
                'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ',
                'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ',
            ].iter()
                .flat_map(|c| c.oomojify())
                .collect::<Vec<char>>(),
            vec![
                'あ', 'い', 'う', 'え', 'お', 'つ', 'や', 'ゆ', 'よ', 'わ',
                'ア', 'イ', 'ウ', 'エ', 'オ', 'ツ', 'ヤ', 'ユ', 'ヨ', 'ワ',
            ]
        );
    }

    #[test]
    fn is_dakuon() {
        assert!(
            [
                'ガ', 'ギ', 'グ', 'ゲ', 'ゴ',
                'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ',
                'ダ', 'ヂ', 'ヅ', 'デ', 'ド',
                'バ', 'ビ', 'ブ', 'ベ', 'ボ',
            ].iter().all(|c| c.clone().is_dakuon())
        );
        assert!(
            [
                'カ', 'キ', 'ク', 'ケ', 'コ',
                'サ', 'シ', 'ス', 'セ', 'ソ',
                'タ', 'チ', 'ツ', 'テ', 'ト',
                'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
            ].iter().all(|c| !c.clone().is_dakuon())
        )
    }

    #[test]
    fn dakuonify() {
        assert_eq!(
            [
                'カ', 'キ', 'ク', 'ケ', 'コ',
                'サ', 'シ', 'ス', 'セ', 'ソ',
                'タ', 'チ', 'ツ', 'テ', 'ト',
                'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
            ]
                .iter()
                .flat_map(|c| c.clone().dakuonify())
                .collect::<Vec<char>>(),
            vec![
                'ガ', 'ギ', 'グ', 'ゲ', 'ゴ',
                'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ',
                'ダ', 'ヂ', 'ヅ', 'デ', 'ド',
                'バ', 'ビ', 'ブ', 'ベ', 'ボ',
            ]
        )
    }
}

pub const KATAKANA_HIRAGANA: [[char; 84]; 2] =
    [
        [
            'ァ', 'ア', 'ィ', 'イ', 'ゥ', 'ウ', 'ェ', 'エ', 'ォ', 'オ', 'カ', 'ガ', 'キ', 'ギ', 'ク',
            'グ', 'ケ', 'ゲ', 'コ', 'ゴ', 'サ', 'ザ', 'シ', 'ジ', 'ス', 'ズ', 'セ', 'ゼ', 'ソ', 'ゾ', 'タ',
            'ダ', 'チ', 'ヂ', 'ッ', 'ツ', 'ヅ', 'テ', 'デ', 'ト', 'ド', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ',
            'バ', 'パ', 'ヒ', 'ビ', 'ピ', 'フ', 'ブ', 'プ', 'ヘ', 'ベ', 'ペ', 'ホ', 'ボ', 'ポ', 'マ', 'ミ',
            'ム', 'メ', 'モ', 'ャ', 'ヤ', 'ュ', 'ユ', 'ョ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ヮ', 'ワ',
            'ヰ', 'ヱ', 'ヲ', 'ン', 'ヴ',
        ],
        [
            'ぁ', 'あ', 'ぃ', 'い', 'ぅ', 'う', 'ぇ', 'え', 'ぉ', 'お', 'か', 'が', 'き', 'ぎ', 'く',
            'ぐ', 'け', 'げ', 'こ', 'ご', 'さ', 'ざ', 'し', 'じ', 'す', 'ず', 'せ', 'ぜ', 'そ', 'ぞ', 'た',
            'だ', 'ち', 'ぢ', 'っ', 'つ', 'づ', 'て', 'で', 'と', 'ど', 'な', 'に', 'ぬ', 'ね', 'の', 'は',
            'ば', 'ぱ', 'ひ', 'び', 'ぴ', 'ふ', 'ぶ', 'ぷ', 'へ', 'べ', 'ぺ', 'ほ', 'ぼ', 'ぽ', 'ま', 'み',
            'む', 'め', 'も', 'ゃ', 'や', 'ゅ', 'ゆ', 'ょ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'ゎ', 'わ',
            'ゐ', 'ゑ', 'を', 'ん', 'ゔ',
        ],
    ];
pub const OOMOJI_KOMOJI: [[char; 10]; 2] = [
    [
        // 'あ', 'い', 'う', 'え', 'お', 'つ', 'や', 'ゆ', 'よ', 'わ',
        'ア', 'イ', 'ウ', 'エ', 'オ', 'ツ', 'ヤ', 'ユ', 'ヨ', 'ワ',
    ],
    [
        // 'ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ', 'っ', 'ゃ', 'ゅ', 'ょ', 'ゎ',
        'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', 'ヮ',
    ],
];
pub const SEION_DAKUON: [[char; 20]; 2] = [
    [
        'カ', 'キ', 'ク', 'ケ', 'コ',
        'サ', 'シ', 'ス', 'セ', 'ソ',
        'タ', 'チ', 'ツ', 'テ', 'ト',
        'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    ],
    [
        'ガ', 'ギ', 'グ', 'ゲ', 'ゴ',
        'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ',
        'ダ', 'ヂ', 'ヅ', 'デ', 'ド',
        'バ', 'ビ', 'ブ', 'ベ', 'ボ',
    ],
];
pub const SEION_HANDAKUON: [[char; 5]; 2] = [
    ['ハ', 'ヒ', 'フ', 'ヘ', 'ホ', ],
    ['パ', 'ピ', 'プ', 'ペ', 'ポ', ]
];

pub trait Kanify: Into<char> + Sized {
    /// ひらがなかどうか判定する
    fn is_hiragana(self) -> bool {
        let chr = self.into();
        KATAKANA_HIRAGANA[1].contains(&chr)
    }

    /// カタカナをひらがな化する
    /// カタカナでない文字を変換しようとした場合は `Err(元の文字)`を返す
    fn hiraganify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_hiragana() {
            return Ok(chr);
        }

        KATAKANA_HIRAGANA[0].binary_search(&chr)
            .map(|index| KATAKANA_HIRAGANA[1][index])
            .map_err(|_| chr)
    }

    /// カタカナかどうか判定する
    fn is_katakana(self) -> bool {
        let chr = self.into();
        KATAKANA_HIRAGANA[0].contains(&chr)
    }

    /// カタカナ化する
    /// ひらがなでない文字を変換しようとした場合は `Err(元の文字)`を返す
    fn katakanify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_katakana() {
            return Ok(chr);
        }

        KATAKANA_HIRAGANA[1].binary_search(&chr)
            .map(|index| KATAKANA_HIRAGANA[0][index])
            .map_err(|_| chr)
    }

    /// 小文字かどうか判定する
    fn is_komoji(self) -> bool {
        let chr = self.into();

        if chr.is_katakana() {
            OOMOJI_KOMOJI[1].contains(&chr)
        } else if chr.is_hiragana() {
            OOMOJI_KOMOJI[1].contains(&chr.katakanify().unwrap())
        } else {
            false
        }
    }

    /// 小文字（捨て仮名）化する
    /// 小文字が存在しない場合は `Err(元の文字)`を返す
    fn komojify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_komoji() {
            return Ok(chr);
        }

        let is_hiragana = chr.is_hiragana();

        let katakana = if is_hiragana {
            chr.katakanify().unwrap()
        } else {
            chr.clone()
        };

        OOMOJI_KOMOJI[0]
            .binary_search(&katakana)
            .map(|index| {
                if is_hiragana {
                    OOMOJI_KOMOJI[1][index]
                        .hiraganify()
                        .unwrap()
                } else {
                    OOMOJI_KOMOJI[1][index]
                }
            })
            .map_err(|_| chr)
    }

    /// 大文字かどうか判定する
    fn is_oomoji(self) -> bool {
        let chr = self.into();

        if chr.is_katakana() {
            OOMOJI_KOMOJI[0].contains(&chr)
        } else if chr.is_hiragana() {
            OOMOJI_KOMOJI[0].contains(&chr.katakanify().unwrap())
        } else {
            false
        }
    }

    /// 大文字化する `komojify()`の逆操作
    fn oomojify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_oomoji() {
            return Ok(chr);
        }

        let is_hiragana = chr.is_hiragana();

        let katakana = if is_hiragana {
            chr.katakanify().unwrap()
        } else {
            chr.clone()
        };

        OOMOJI_KOMOJI[1]
            .binary_search(&katakana)
            .map(|index| {
                if is_hiragana {
                    OOMOJI_KOMOJI[0][index]
                        .hiraganify()
                        .unwrap()
                } else {
                    OOMOJI_KOMOJI[0][index]
                }
            })
            .map_err(|_| chr)
    }

    /// 濁音かどうか判定する
    fn is_dakuon(self) -> bool {
        let chr = self.into();

        if chr.is_katakana() {
            SEION_DAKUON[1].contains(&chr)
        } else if chr.is_hiragana() {
            SEION_DAKUON[1].contains(&chr.katakanify().unwrap())
        } else {
            false
        }
    }

    /// 濁音化する
    /// 濁音が存在しない場合は `Err(元の文字)`を返す
    fn dakuonify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_dakuon() {
            return Ok(chr);
        }

        let is_hiragana = chr.is_hiragana();

        let katakana = if is_hiragana {
            chr.katakanify().unwrap().seionify().unwrap()
        } else if chr.is_katakana() {
            chr.clone().seionify().unwrap()
        } else {
            return Err(chr);
        };

        SEION_DAKUON[0]
            .binary_search(&katakana)
            .map(|index| {
                if is_hiragana {
                    SEION_DAKUON[1][index]
                        .hiraganify()
                        .unwrap()
                } else {
                    SEION_DAKUON[1][index]
                }
            })
            .map_err(|_| chr)
    }

    /// 半濁音かどうか判定する
    fn is_handakuon(self) -> bool {
        let chr = self.into();

        if chr.is_katakana() {
            SEION_HANDAKUON[1].contains(&chr)
        } else if chr.is_hiragana() {
            SEION_HANDAKUON[1].contains(&chr.katakanify().unwrap())
        } else {
            false
        }
    }

    /// 半濁音化する
    /// 半濁音が存在しない場合は `Err(元の文字)`を返す
    fn handakuonify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_handakuon() {
            return Ok(chr);
        }

        let is_hiragana = chr.is_hiragana();

        let katakana = if is_hiragana {
            chr.katakanify().unwrap().seionify().unwrap()
        } else if chr.is_katakana() {
            chr.clone().seionify().unwrap()
        } else {
            return Err(chr);
        };

        SEION_HANDAKUON[0].binary_search(&katakana)
            .map(|idx| if is_hiragana {
                SEION_HANDAKUON[1][idx]
                    .hiraganify()
                    .unwrap()
            } else {
                SEION_HANDAKUON[1][idx]
            })
            .map_err(|_| chr)
    }

    /// 清音かどうか判定する
    fn is_seion(self) -> bool {
        let chr = self.into();

        // ひらがなもしくはカタカナで、
        (chr.is_hiragana() || chr.is_katakana())
            // 濁音ではなく、
            && !chr.is_dakuon()
            // 半濁音でもない
            && !chr.is_handakuon()
    }

    /// 清音化する
    /// 清音が存在しない場合は `Err(元の文字)`を返す
    fn seionify(self) -> Result<char, char> {
        let chr = self.into();

        if chr.is_seion() {
            return Ok(chr);
        }

        let is_hiragana = chr.is_hiragana();

        let katakana = if is_hiragana {
            chr.katakanify().unwrap()
        } else if chr.is_katakana() {
            chr.clone()
        } else {
            return Err(chr);
        };

        if chr.is_dakuon() {
            Ok(
                SEION_DAKUON[1]
                    .binary_search(&katakana)
                    .map(|idx| if is_hiragana {
                        SEION_DAKUON[0][idx].hiraganify().unwrap()
                    } else {
                        SEION_DAKUON[0][idx]
                    })
                    .unwrap()
            )
        } else if chr.is_handakuon() {
            Ok(
                SEION_HANDAKUON[1]
                    .binary_search(&katakana)
                    .map(|idx| if is_hiragana {
                        SEION_HANDAKUON[0][idx].hiraganify().unwrap()
                    } else {
                        SEION_HANDAKUON[0][idx]
                    })
                    .unwrap()
            )
        } else {
            Err(chr)
        }
    }
}

impl<T: Into<char>> Kanify for T {}