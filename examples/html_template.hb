<h1>{{this.name}}</h1>

<h2>Messages</h2>

{{#each messages}}
    <div>
        <h3>{{this.name}}</h3>
        <ul>
            {{#each this.fields}}
                <li>
                    {{this.label}} {{this.type}} {{this.name}} = {{this.number}}</li>
            {{/each}}
        </ul>
    </div>
{{/each}}

<h2>Services</h2>

{{#each services}}
    <div>
        <h3>{{this.name}}</h3>
        <ul>
            {{#each this.methods}}
                <li>{{this.name}} {{this.input_type}} {{this.output_type}}</li>
            {{/each}}
        </ul>
    </div>
{{/each}}