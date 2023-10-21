-- Add migration script here

create table if not exists events_general_info
(
    id integer primary key,
    timestamp integer not null,
    user_id integer not null,
    guild_id integer not null
);

create table if not exists events_voice_state_update
(
    id integer primary key,
    general_event_info_id integer not null
        references events_general_info (id),
    voice_channel_id integer null,
    self_muted boolean,
    self_deafened boolean,
    self_streaming boolean,
    self_video boolean,
    deafened boolean,
    muted boolean,
    suppressed boolean
);

create table if not exists events_message
(
    id integer primary key,
    general_event_info_id integer not null
        references events_general_info (id),
    text_channel_id integer not null,
    message_id integer not null,
    content text not null
);



