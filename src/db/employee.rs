use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::employee::{Employee, NewEmployee, UpdateEmployee},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct EmployeeRepository {
    pool: Arc<PgPool>,
}

impl EmployeeRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_employee, hashing))]
    pub async fn create(
        &self,
        id_user: String,
        new_employee: NewEmployee,
        hashing: &CryptoService,
    ) -> Result<Employee> {
        let password_hash = hashing.hash_password(new_employee.password).await?;

        let employee = sqlx::query_as::<_, Employee>(
            "insert into employees (name, email, password_hash) values ($1, $2, $3) returning *",
        )
        .bind(new_employee.name)
        .bind(new_employee.email)
        .bind(password_hash)
        .fetch_one(&*self.pool)
        .await?;

        println!("{:?}", employee);
        Ok(employee)
    }

    pub async fn update_employee(
        &self,
        id_employee: String,
        employee: UpdateEmployee,
        hashing: &CryptoService,
    ) -> Result<Employee> {
        let id_employee = uuid::Uuid::parse_str(&id_employee)?;
        let password_hash = hashing.hash_password(employee.password).await?;

        let employee = sqlx::query_as::<_, Employee>(
            "update employees set name = $2, password_hash = $3 where id = $1 returning *",
        )
        .bind(id_employee)
        .bind(employee.name)
        .bind(password_hash)
        .fetch_one(&*self.pool)
        .await?;
        Ok(employee)
    }

    #[instrument(skip(self))]
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Employee>> {
        let maybe_employee =
            sqlx::query_as::<_, Employee>("select * from employees where name = $1")
                .bind(name)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(maybe_employee)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id_employee: String) -> Result<Option<Employee>> {
        let id_employee = uuid::Uuid::parse_str(&id_employee)?;
        let maybe_employee = sqlx::query_as::<_, Employee>("select * from employee where id = $1")
            .bind(id_employee)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(maybe_employee)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id_user: Uuid) -> Result<Vec<Employee>> {
        let mut all_employees = vec![];

        let result = sqlx::query!(
            r#"
                SELECT *
                FROM employees
                where id_user = $1"#,
            id_user
        )
        .fetch_all(&*self.pool)
        .await?;

        for employee in result {
            all_employees.push(Employee {
                id: employee.id,
                name: employee.name,
                email: employee.email,
                password_hash: employee.password_hash,
                email_verified: employee.email_verified,
                active: employee.active,
                created_at: employee.created_at,
                updated_at: employee.updated_at,
                id_organization: employee.id_organization,
                id_user: employee.id_user,
                position: employee.position,
            })
        }

        Ok(all_employees)
    }
}

impl FromRequest for EmployeeRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(EmployeeRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
