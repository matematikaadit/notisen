# Notisen

**Notisen** is a Rust rewrite of [highmon.pl] script for [weechat] irc client.
Proudly with less feature and less flexibility.

**highmon.pl** itself is a script that collect all *highlights* (another term
for mention/notification on weechat). In a single buffer for easy viewing. This
script only replicate that functionality and no more. There's no setting or
command hook, yeah, sorry to disssapoint.

This project itself is mainly for learning purpose. To understand how to do FFI
in Rust. And probably just a toy that incidentally also scratch my own itch.

If you wanna write a plugin for weechat, I recommend using the rust-weechat
crate instead.  Link in the [**Resource**](#resource) section below.

Currently this plugin is already working. And I've been using it for at least
one week without any problem. If you see any possible issue, don't afraid to
open new issue/pull request on github. Any feedback is highly appreciated.

# The Name

**Notisen** is short for *Notice Me Senpai*.

# Compiling and Installing

Require Rust, Bindgen, and Weechat installed. Execute the following to compile
and install the plugin into your weechat's plugin folder
(`$HOME/.weechat/plugins`).

```
make
make install
```

# Safe Abstraction for the Plugin API

This section is a part of the recent rewrite. A design idea for the plugin API.
Will be expanded if the rewrite succesful.

* TODO: explain the current C API from Weechat.
* TODO: explain the new design on top of that glue layer.

# Todo/Future Improvement

* Write tests.
* Make it works for any version of weechat.

# Resource

* [rust-weechat crate](https://github.com/poljar/rust-weechat)
* [bindgen](https://github.com/rust-lang/rust-bindgen)
* [writing -sys crate](https://kornel.ski/rust-sys-crate)
* [c2rust](https://c2rust.com/)
* [highmon.pl](https://github.com/KenjiE20/highmon/)
* [weechat's plugin api](https://weechat.org/files/doc/devel/weechat_plugin_api.en.html)

# License

Copyright (c) 2019 by Adit Cahya Ramadhan <matematika.adit@gmail.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.


[highmon.pl]: https://github.com/KenjiE20/highmon
[weechat]: https://weechat.org/
