# tinyproto

**What is this?**

`tinyproto` is a `.proto` file parser written in Rust. It can do two things:

* Convert `.proto` files to a JSON tree
* Use `.proto` files as input to a handlebars template

**Why is this helpful?**

It makes it possible to build your own "code generator" by writing a handlebars
template file. Before this, you'd need to write a plugin for `protoc` itself, which
I found difficult and time-consuming.

Note, this parser only parses a subset of the proto spec. You can find the [EBNF grammer for proto3 here.](https://developers.google.com/protocol-buffers/docs/reference/proto3-spec#reserved) I find this subset to be sufficient. If you run into any big issues though, create an issue or feature request! Happy to address them.

Some ideas of how this might be used:

* Define your API in proto and generate strongly-typed backend and frontend stubs
* Generate HTML documentation of your API
* Use proto as a strongly-typed schema for JSON objects, and generate methods for this

### How is it implemented?

The tool is written entirely in Rust. It features a zero-allocation recursive descent parser written with `nom`.

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
