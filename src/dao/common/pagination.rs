#[cfg(test)]
use fake::Dummy;
use thiserror::Error;

#[cfg_attr(test, derive(Debug, Dummy, PartialEq, Eq))]
#[derive(Clone)]
pub struct Pagination {
    #[cfg_attr(test, dummy(faker = "1..10"))]
    page: usize,
    #[cfg_attr(test, dummy(faker = "1..10"))]
    limit: usize,
}

impl Pagination {
    pub fn page(&self) -> usize {
        self.page
    }

    pub fn limit(&self) -> usize {
        self.limit
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug))]
pub struct PaginationBuilder {
    page: usize,
    limit: usize,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PaginationBuilderError {
    #[error("Page number must be greater than zero")]
    PageIsZero,
    #[error("Entites limit must be greater than zero")]
    LimitIsZero,
}

impl Default for PaginationBuilder {
    fn default() -> Self {
        Self {
            page: Self::DEFAULT_PAGINATION_PAGE,
            limit: Self::DEFAULT_PAGINATION_LIMIT,
        }
    }
}

impl PaginationBuilder {
    const DEFAULT_PAGINATION_LIMIT: usize = 10;
    const DEFAULT_PAGINATION_PAGE: usize = 1;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn build(self) -> Result<Pagination, PaginationBuilderError> {
        if self.page.eq(&0) {
            return Err(PaginationBuilderError::PageIsZero);
        }

        if self.limit.eq(&0) {
            return Err(PaginationBuilderError::LimitIsZero);
        }

        Ok(Pagination {
            page: self.page,
            limit: self.limit,
        })
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use super::*;

    #[test]
    fn default_builder() {
        let builder = PaginationBuilder::default();
        println!("{builder:#?}");
        let pagination = builder.build().unwrap();
        println!("{pagination:#?}");

        assert_eq!(
            pagination,
            Pagination {
                page: PaginationBuilder::DEFAULT_PAGINATION_PAGE,
                limit: PaginationBuilder::DEFAULT_PAGINATION_LIMIT
            }
        );
    }

    #[test]
    fn zero_page_builder() {
        let builder_err = PaginationBuilder::new().page(0).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(PaginationBuilderError::PageIsZero));
    }

    #[test]
    fn zero_limit_builder() {
        let builder_err = PaginationBuilder::new().limit(0).build();
        println!("{builder_err:#?}");

        assert_eq!(builder_err, Err(PaginationBuilderError::LimitIsZero));
    }

    #[test]
    fn builder() {
        let page: usize = (1..10).fake();
        let limit: usize = (1..10).fake();

        let pagination = PaginationBuilder::new()
            .limit(limit)
            .page(page)
            .build()
            .unwrap();
        println!("{pagination:#?}");

        assert_eq!(pagination.page(), page);
        assert_eq!(pagination.limit(), limit);
    }
}
