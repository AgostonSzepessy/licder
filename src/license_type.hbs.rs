pub enum LicenseType {
    {{#each licenses}}
        {{~@key}},
    {{/each}}
}
