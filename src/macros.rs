macro_rules! cfg_stream {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "stream")]
            #[cfg_attr(feature = "_docs", doc(cfg(feature = "stream")))]
            $item
        )*
    }
}

macro_rules! cfg_unix {
    ($($item:item)*) => {
        $(
            #[cfg(any(unix, feature = "_docs"))]
            #[cfg_attr(feature = "_docs", doc(cfg(unix)))]
            $item
        )*
    }
}
