# Shellbookmark

Shellbookmark is a simple key-value store based bookmark tracker.
!!! Change main.rs to your own home directory !!!
```
static STOREPATH: &'static str = "/home/asui/.config/pathstore";
```

## Usage

Add a new store-entry using the --set option:
```
cd /path/to/important/folder
shellbookmark -s important
```

Now you have an entry in your store that looks like:
```
/path/to/important/folder     important
```


You can query the store using `shellbookmark -g important`


## Aliases

To unlock the whole power of shellbookmark you should use it with aliases:

```
alias sp="shellbookmark -s" 
alias pp="shellbookmark -p"

gp() {
	cd $(shellbookmark -g $1)
}
```


