```pwsh
go run . -gua-list-fmt "- `"%s`"`n" .\custom.txt
```


```pwsh
go run . -gua-list-fmt "- name: `"%s`"`n  type: `"server`"`n  opts: `"{{ ntp_master_list_opts }}`"`n" .\custom.txt
```
