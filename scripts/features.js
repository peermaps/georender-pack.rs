var fs = require('fs')
var data = JSON.parse(fs.readFileSync(process.argv[2],'utf8'))
console.log(`
  use std::collections::HashMap;

  pub fn get_types<'a>() -> HashMap<&'a str, u64> {
      return [\n${Object.keys(data).map(key => {
          return `          (${JSON.stringify(key)}, ${data[key]}),`
      }).join('\n')}
      ]
      .iter()
      .map(|tup| (tup.0, tup.1 as u64))
      .collect();
}`.replace(/^ {2}/mg,'').replace(/^\s+/,''))
