# tinyproto

`tinyproto` is a `.proto` file parser written in Rust. It can do two things:

* Convert `.proto` files to a JSON tree
* Pass this JSON tree into a handlebars template

This makes it *very* easy to generate custom code from protobuf files!
I use this to generate backend and frontend API stubs (to keep the types in sync),
but this could also be used to generate HTML for documentation of your API.

Note, this parser only parses a subset of the proto spec. You can find the [EBNF grammer for proto3 here.](https://developers.google.com/protocol-buffers/docs/reference/proto3-spec#reserved) If you want to change this, contributions are welcome!

### Why does this exist?

I find `protoc` pretty non-trivial to install, use and extend. Especially if you don't
want to use the default generators, or if one doesn't exist for your language. Also, sometimes
I want to use `.proto` files without the wire format.

This allows me to generate some code in the ~10mins it takes to write a handlebars template file.

### Usage

#### To print JSON

1. Write a `.proto` file
2. Run `tinyproto --proto <path to proto file> `

#### To generate from a template

1. Write a `.proto` file
2. Write a handlebars template file
3. Run `tinyproto --proto <path to proto file> --template <path to template file>`

### Examples

See the `/examples` folder.

This `.proto` file:

```proto
# service.proto
message MyMessage {
  required string id = 1;
}
```

Will generate this JSON with `tinyproto --proto service.proto`:

```
{
  "name": "service.proto",
  "syntax": null,
  "messages": [
    {
      "name": "MyMessage",
      "fields": [
        {
          "name": "id",
          "type": "string",
          "label": "required",
          "number": "1",
        }
      ]
    }
  ],
  "services": []
}
```

And when used with the following `template.hb` handlebars file:

```hb
<h1>{{name}}</h1>
<ul>
{{#each messages}}
  <li>{{this.name}}</li>
{{/each}}
</ul>
```

After running `tinyproto --proto service.proto --template template.hb`, we get:

```html
<h1>service.proto</h1>
<ul>
  <li>MyMessage</li>
</ul>
```
