/// Font variant for [`<mi>`](https://developer.mozilla.org/en-US/docs/Web/MathML/Element/mi),
/// used as [`mathvariant`](https://developer.mozilla.org/en-US/docs/Web/MathML/Global_attributes/mathvariant) attribute.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MathVariant {
    /// <math><mi mathvariant="normal">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    Nothing,
    /// <math><mi mathvariant="normal">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathrm{}`
    Normal,
    /// <math><mi mathvariant="italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathit{}`
    Italic,
    /// <math><mi mathvariant="bold">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbf{}`
    Bold,
    /// <math><mi mathvariant="bold-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbf{\mathit{}}`
    BoldItalic,
    /// <math><mi mathvariant="double-struck">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbb{}`
    DoubleStruck,
    /// <math><mi mathvariant="fraktur">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathfrak{}`
    Fraktur,
    /// <math><mi mathvariant="bold-fraktur">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbf{\mathfrak{}}`
    BoldFraktur,
    /// <math><mi mathvariant="script">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathscr{}`
    Script,
    /// <math><mi mathvariant="bold-script">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbf{\mathscr{}}`
    BoldScript,
    /// `\mathcal{}`
    Calligraphy,
    /// <math><mi mathvariant="sans-serif">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathsf{}`
    SansSerif,
    /// <math><mi mathvariant="bold-sans-serif">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbf{\mathsf{}}`
    BoldSansSerif,
    /// <math><mi mathvariant="sans-serif-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathsf{\mathit{}}`
    SansSerifItalic,
    /// <math><mi mathvariant="sans-serif-bold-italic">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathbf{\mathsf{\mathit{}}}`
    SansSerifBoldItalic,
    /// <math><mi mathvariant="monospace">ABCDEFGHIJKLMNOPQRSTUVWXYZ</mi></math>
    ///
    /// `\mathtt{}`
    Monospace,
}
