# HyperCortex

## HyperTask

### Todo

-   [1Hr] create cli package with storage config
-   [2Hr] hook up old rendering functionality
-   [5Hr] build conversational UI with `Inquirer.js`
    -   [2Hr] question based `add` command
    -   [2Hr] selection based `modify`, `start`, `stop`, `delete`, `done` commands
    -   [1Hr] final confirm dialog
-   [2Hr] `hyperdb` replication logic
-   [1Hr] `hyperdb` reducer logic

### Desired CLI API

```bash
#admin and config
$ task setup [existing hypercortex key]
$ task auth [other local hypercortex key]
$ task share

#creation
$ task add take out the trash +chores due:now
$ task id modify wait:1d priority:H

#workflow shortcuts
$ task id start
$ task id stop
$ task id done
$ task id delete
```
