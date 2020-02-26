macro_rules! cfg_docs {
    (
        $(#[doc = $doc:expr])*
        #[cfg($cfg:meta)]
        $vis:vis fn $($rest:tt)+
    ) => {
        $(#[doc = $doc])*
        #[cfg(any(docsrs, $cfg))]
        #[cfg_attr(docsrs, doc(cfg($cfg)))]
        $vis fn $($rest)+
    };
}
