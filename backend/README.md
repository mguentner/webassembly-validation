minimal rust backend using the types from `shared` accepting
`application/json` content on `http://127.0.0.1:3000/hosts` using POST.

The main idea is that any other client within this sample project attempts
to create a host but wants to validate the payload first in order to
make the interface more responsive or avoid requests chains that contain
avoidable errors.

Run this with

`cargo run`

You can find [bruno](https://www.usebruno.com/) requests in `requests` if you
want to try out the API first.