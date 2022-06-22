<p align="center">
<img src="qubist_white.png#gh-dark-mode-only" width="300"></img>
<img src="qubist_black.png#gh-light-mode-only" width="300"></img>
<br/>
</p>

[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#experimental)

> Language-agnostic blackbox testing

```ruby
# Hello, World!
Base URL is http://localhost:8080
Test BasicExample
    Test HTTPSample
        GET /test returns `test`
    End
    # This is a comment
    Test CLISample
        Executing `
            printf "Hello, "
            printf "World!"
        ` returns `Hello, World!`
    End
End
```
Qubist is a language-agnostic blackbox testing tool, using a domain-specific language to define specifications.
