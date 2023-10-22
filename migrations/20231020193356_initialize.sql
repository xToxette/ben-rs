-- Add migration script here
create table if not exists events_general_info
(
    id integer primary key not null,
    timestamp integer not null,
    user_id integer not null,
    guild_id integer null
);

create table if not exists events_voice_state_update
(
    id integer primary key not null ,
    general_event_info_id integer not null
        references events_general_info (id),
    voice_channel_id integer null,
    self_muted boolean not null,
    self_deafened boolean not null,
    self_streaming boolean not null,
    self_video boolean not null,
    deafened boolean not null,
    muted boolean not null,
    suppressed boolean not null
);

create table if not exists events_message
(
    id integer primary key not null,
    general_event_info_id integer not null
        references events_general_info (id),
    text_channel_id integer not null,
    message_id integer not null,
    content text not null
);



