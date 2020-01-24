macro_rules! cfg_stream {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "stream")]
            #[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
            $item
        )*
    }
}

macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(any(unix, docsrs))]
            #[cfg_attr(docsrs, doc(cfg(unix)))]
            $item
        )*
    }
}
