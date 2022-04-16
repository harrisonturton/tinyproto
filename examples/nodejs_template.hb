const express = require('express')
const app = express()
const port = 3000

{{#each services}}
let {{this.name}} = new {{this.name}}();
{{/each}}

app.get('/', (req, res) => {
  res.send('Hello World!')
})

{{#each services}}
{{#each this.methods}}
app.get('/{{ this.name }}', (req, res) => {
  var response = {{../name}}.{{this.name}}()
  res.send(response) 
})

{{/each}}
{{/each}}
app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})

{{#each services}}
class {{this.name}} {
    {{#each this.methods}}

    function {{this.name}}(req) {
        return "You invoked {{this.name}}!"
    }
    {{/each}}
}
{{/each}}