use crate::m20240212_023838_create_post_table::Post::{self};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Create Foreign Key
                    .col(ColumnDef::new(Comment::ForeignId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post_id")
                            .to(Post::Table, Post::Id)
                            .from(Comment::Table, Comment::ForeignId),
                    )
                    .col(ColumnDef::new(Comment::Title).string().not_null())
                    .col(ColumnDef::new(Comment::Content).string().not_null())
                    .col(ColumnDef::new(Comment::Author).string().default("Anonymous"))
                    .col(
                        ColumnDef::new(Comment::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Comment::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    ForeignId,
    Title,
    Content,
    Author,
    CreatedAt,
    UpdatedAt,
}
