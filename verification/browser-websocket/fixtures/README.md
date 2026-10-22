# Public synthetic TLS fixtures

These local-only certificates and the deliberately public disposable test key
come from this repository's WASI socket proof. They identify only `localhost`
and `kit-local-test-ca`, have no external account or production use, and remain
under this project's Unlicense. The leaf has only a DNS localhost subjectAltName
so a connection by 127.0.0.1 proves destination-name rejection. Certificates are
valid from 2026-10-22 to 2036-10-19. No machine trust store or personal key is read.
