# shleazy

"Shell Made Easy"

There are several `std::process` wrappers, but all seem too verbose for
some common use cases.

`_shell` variants wrap command in `sh -c ''`.

## Examples

```rust
use shleazy::*;

fn main() -> Result<()> {

    // Returns Err since non-zero exit code
    run_shell_or_err("ls /invalid-path")?;

    // Returns 1
    let exit_code = getstatus_shell("false")?;

    // Returns "test"
    let output = getoutput_shell_or_err("echo 'test'")?;

    // returns (0, "test")
    let (exit_code, output) = getstatusoutput_shell("echo 'test'")?;
}

```


## TODO

* flesh out all combinations (existing implemented as needed)
* capture/combine `stderr`
* model around a struct instead? (Seems more verbose?), for example:

```rust
Cmd::new("").
Cmd::new_shell("").
Cmd::new_args("", "arg1").
Cmd::new("").output()
Cmd::new("").output_or_err()
Cmd::new("").status()
Cmd::new("").status_or_err()
Cmd::new("").or_err()
Cmd::new("").statusoutput()
Cmd::new("").statusoutput_or_err()
```
