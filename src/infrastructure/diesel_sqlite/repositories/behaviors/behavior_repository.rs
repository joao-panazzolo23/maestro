use diesel_async::pooled_connection::deadpool::Pool;

use crate::{
    domain::behaviors::{
        entities::behavior::Behavior,
        repositories::behavior_repository::{BehaviorRepository, RepositoryError},
    },
    infrastructure::async_sqlite_conn::AsyncSqliteConnection,
};

pub struct DieselBehaviorRepository {
    pool: Pool<AsyncSqliteConnection>,
}

impl BehaviorRepository for DieselBehaviorRepository {
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn create<'life0, 'async_trait>(
        &'life0 self,
        behavior: Behavior,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), RepositoryError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn delete<'life0, 'async_trait>(
        &'life0 self,
        behavior: Behavior,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), RepositoryError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn update<'life0, 'async_trait>(
        &'life0 self,
        behavior: Behavior,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<(), RepositoryError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn get<'life0, 'async_trait>(
        &'life0 self,
        id: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Behavior, RepositoryError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}
