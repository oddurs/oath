# Security

## Supported versions

Oath is pre-release. Only the default branch is supported, and there are no security
backports to earlier tags.

| Version | Supported |
|---|---|
| `main` | yes |
| everything else | no |

## Reporting a vulnerability

Report privately through GitHub Security Advisories:
[open a draft advisory](https://github.com/oddurs/oath/security/advisories/new).

Please do not open a public issue for a vulnerability.

Expect acknowledgement within a week. This is a hobby project maintained by one person,
so a fix may take longer than that, and you will be told where it stands rather than
left waiting. If you would like credit in the advisory, say so and how you want to be
named.

## Scope

Oath swears keepers by running them, including keepers proposed by an external command.
Sandboxing that path is a known open problem tracked in the backlog rather than a
vulnerability: running untrusted Oath source is not safe yet, and the documentation says
so. Reports that sharpen *how* it is unsafe are welcome.
