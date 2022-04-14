# tinyproto

`tinyproto` is a small commandline tool that does two things:

* Convert `.proto` files to a JSON tree
* Pass this JSON tree into a handlebars template

This makes it *very* easy to generate custom code from protobuf files!
I use this to generate backend and frontend API stubs (to keep the types in sync),
but this could also be used to generate HTML for documentation of your API.

### Why does this exist?

I find `protoc` pretty non-trivial to install, use and extend. Especially if you don't
want to use the default generators, or if one doesn't exist for your language. Also, sometimes
I want to use `.proto` files without the wire format.

This allows me to generate some code in the ~10mins it takes to write a handlebars template file.

### Usage

1. Write a `.proto` file
2. Write a handlebars template file
3. Run `tinyproto --proto <path to proto file> --template <path to template file>`


### Examples

See the `/examples` folder.
