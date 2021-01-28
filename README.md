# intbak
utility for backing up wow interfaces. intended to be used on win10 systems for now.
when running tests, use `--test-threads=1` because the tests for this utility revolve around fs reading / writing and running tests in async can run things out of order.
# todo
- prompt user for target directory
- add list of folders to backup
- recursively pull contents
- zip payload
- identify backup location
- store payload
- add metadata about backup, eg. date, time
- integrate with a system dialog window for better user friendliness