create table tbl_exflow_app
(
    app_id     varchar(255)               not null
        constraint tbl_exflow_app_pk
            primary key,
    app_name   varchar(255)               not null,
    app_detail varchar(255)               not null,
    created_dt datetime default getdate() not null,
    updated_dt datetime default getdate(),
    updated_by varchar(255)
);

create table tbl_exflow_runtime_clients
(
    client_id varchar(255) not null,
    host_name varchar(255) not null,
    host_ip   varchar(15)  not null,
    constraint tbl_exflow_runtime_clients_pk
        primary key (client_id, host_name, host_ip)
);


