
REQUEST='{"input_folder":"/home/julianramirezj/backup-system-api/backup/input-backup-2","output_folder":"/home/julianramirezj/output-backup","pass":"secret"}'
curl -v -i -X POST -H 'Content-Type: application/json' 'http://127.0.0.1:8000/backup/restore' -d "${REQUEST}"
