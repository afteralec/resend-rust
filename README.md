# This Resend Client is a Work In Progress and Being Actively Developed

Hi! This [Resend](https://resend.com) client is currently a work in progress
and being actively developed.

Yes, `resend-rs` already exists but doesn't implement the entire surface
area of Resend's API as found in an example like the [Resend Go client](https://github.com/resendlabs/resend-go).
This is an effort to develop a relatively fully-functioning Resend client for Rust.

## Update as of v0.1.1

The overall API for everything except anything currently in beta is implemented.

Everything except the Domains API, which requires a paid account, is under
integration test.

Nothing is documented yet; I'm polishing some of the surface area
of the APIs, then wrapping those up.
