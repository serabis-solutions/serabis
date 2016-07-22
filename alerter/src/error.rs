
quick_error! {
    #[derive(Debug)]
    pub enum AlerterError {
        DbConnection(err: ::postgres::error::ConnectError) {
            from()
            description(err.description())
            cause(err)
        }

        Pool(err: ::r2d2::InitializationError) {
            from()
            description(err.description())
            cause(err)
        }
    }
}
