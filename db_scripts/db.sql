create table tbl_exflow_app
(
    app_id     varchar(255)               not null
        constraint tbl_exflow_app_pk
            primary key,
    app_name   varchar(255)               not null,
    app_detail varchar(255)               not null,
    created_dt datetime default getdate() not null,
    updated_dt datetime default getdate() not null,
    updated_by varchar(255)
);

create table tbl_exflow_job_history
(
    app_id     varchar(255) not null,
    job_id     varchar(255) not null,
    run_id     varchar(255) not null,
    status     varchar(15)  not null,
    message    nvarchar(max),
    created_dt datetime default getdate(),
    constraint tbl_exflow_job_history_pk
        primary key (app_id, job_id, run_id)
);

create table tbl_exflow_jobs
(
    app_id varchar(255)
        constraint tbl_exflow_jobs_tbl_exflow_app_app_id_fk
            references tbl_exflow_app,
    job_id varchar(255) not null
);

create unique index tbl_exflow_jobs_app_id_job_id_uindex
    on tbl_exflow_jobs (app_id, job_id);

create table tbl_exflow_runtime_clients
(
    client_id varchar(255) not null,
    host_name varchar(255) not null,
    host_ip   varchar(15)  not null,
    constraint tbl_exflow_runtime_clients_pk
        primary key (client_id, host_name, host_ip)
);
