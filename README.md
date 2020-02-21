# moko

moko is very simple file sharing web app.

It's like a SandBox for building web applications in Rust, using Actix, barrel, diesel... and etc.
I developing based on DDD design.

At this time, i cannot test what works, because the compilation of `diesel` fails.

## design

⚠ **These configurations and content are subject to change.**

- app: describes the operation logic of the application.

- data: describes the logic related to the data model and the DB logic.

- domain: describes the logic related to the domain model.

- migration: migration logic using `barrel` is written.
