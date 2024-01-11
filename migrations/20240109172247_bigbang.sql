create table labels (
    id          uuid                     default gen_random_uuid() not null primary key,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null,
    name        text                                               not null unique,
    description text,
    color       varchar
);

create trigger set_public_labels_updated_at
    before update
    on labels
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_labels_updated_at on labels is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table labels_by_tasks (
    label_id uuid not null,
    task_id  uuid not null,
    primary key (label_id, task_id)
);

create table members (
    id            uuid                     default gen_random_uuid() not null primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    name          text                                               not null,
    email         varchar                                            not null,
    password_hash varchar,
    github_id     varchar unique,
    google_id     varchar unique,
    photo_url     varchar,
    role          varchar
);

create trigger set_public_members_updated_at
    before update
    on members
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_members_updated_at on members is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table members_by_projects (
    member_id  uuid not null,
    project_id uuid not null,
    primary key (member_id, project_id)
);

create table members_by_teams (
    team_id   uuid not null,
    member_id uuid not null,
    role      varchar default 'Member'::character varying,
    primary key (team_id, member_id)
);

create table projects (
    id          uuid                     default gen_random_uuid() not null primary key,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null,
    name        text                                               not null,
    prefix      varchar,
    owner_id    uuid                                               not null references members on update cascade on delete set null,
    description text,
    lead_id     uuid,
    start_date  timestamp with time zone,
    due_date    timestamp with time zone,
    status      varchar,
    visibility  varchar
);

create trigger set_public_projects_updated_at
    before update
    on projects
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_projects_updated_at on projects is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table self (
    id         uuid                     default gen_random_uuid() not null primary key,
    created_at timestamp with time zone default now()             not null,
    updated_at timestamp with time zone default now()             not null,
    name       text                                               not null
);

create trigger set_public_self_updated_at
    before update
    on self
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_self_updated_at on self is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table tasks (
    id          uuid                     default gen_random_uuid() not null primary key,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null,
    title       text                                               not null,
    description text,
    owner_id    uuid                                               not null references members on update cascade on delete set null,
    status      varchar,
    priority    varchar,
    due_date    timestamp with time zone,
    project_id  uuid,
    lead_id     uuid,
    labels      jsonb,
    count       serial,
    parent_id   uuid
);

create trigger set_public_tasks_updated_at
    before update
    on tasks
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_tasks_updated_at on tasks is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table tasks_by_assignees (
    task_id     uuid not null references tasks on update cascade on delete cascade,
    assignee_id uuid not null references members on update cascade on delete cascade,
    primary key (task_id, assignee_id)
);

create table tasks_by_projects (
    task_id    uuid not null constraint tasks_by_projects_task_fkey references tasks on update cascade on delete cascade,
    project_id uuid not null constraint tasks_by_projects_project_fkey references projects on update cascade on delete cascade,
    primary key (task_id, project_id)
);

create table teams (
    id         uuid                     default gen_random_uuid() not null primary key,
    created_at timestamp with time zone default now()             not null,
    updated_at timestamp with time zone default now()             not null,
    name       varchar                                            not null,
    owner_id   uuid                                               not null,
    visibility varchar,
    prefix     text unique
);

create trigger set_public_teams_updated_at
    before update
    on teams
    for each row
execute procedure set_current_timestamp_updated_at();

comment on trigger set_public_teams_updated_at on teams is 'trigger to set value of column "updated_at" to current timestamp on row update';

create table teams_by_projects (
    team_id    uuid not null,
    project_id uuid not null,
    primary key (team_id, project_id)
);

create table activity (
    id            uuid                     default gen_random_uuid() not null primary key,
    created_at    timestamp with time zone default now()             not null,
    updated_at    timestamp with time zone default now()             not null,
    member_id     uuid                                               not null references members on delete cascade,
    resource_id   uuid                                               not null,
    operation     text                                               not null,
    resource_type text                                               not null
);

create index activity_member_id_idx
    on activity (member_id);

create index activity_resource_id_idx
    on activity (resource_id);

