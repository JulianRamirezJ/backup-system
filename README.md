# Backup System
## Por: Julian Andres Ramirez Jimenez
### Repositorio del proyecto: https://github.com/JulianRamirezJ/backup-system.git

## Descripción del proyecto

Este proyecto consistió en el desarrollo de un sistema que permite hacer backups de carpetas completas con todos sus archivos
mediante el uso del lenguaje Rust y algunas librerias que sirvieron de apoyo. El sistema tiene a grandes rasgos tiene dos funcionalidades 
importantes, que son crear un backup y restaurar un backup. A continuación se exponen estos dos modulos.

Crear un backup consta de tomar la carpeta a guardar, poner todos sus archivos en la ruta del backup comprimidos en 
un formato tar, luego dividir este archivo tar en segmentos según el peso del archivo y por ultimo encriptar de forma paralela los
archivos resultantes. En este proceso también se crea un archivo json que guarda información sobre los segmentos del archivo,en 
que orden restaurarlos y una contraseña para el backup que se debe poner a la hora de restaurar.

Restaurar un backup consta de tomar la carpeta donde se creó el backup,verificar la contraseña del backup que provee el usuario, posteriormente desencriptar todos sus archivos de forma paralela. Luego rearmar el tar según el orden indicado en el archivo json y por último descomprimir 
el tar en la ruta indicada por el usuario, así se devolveria la carpeta a su estado original. Una anotación importante es que luego de restaurar la carpeta, continuan estando los mismos archivos encriptados que se tenian al crear el backup, ya que los desencriptados y el bloque completo del tar se eliminan inmediatamente se termina de operar con ellos, garantizando asi la seguridad de los datos del usuario.


## Como compilar y ejecutar

Para ejecutar el programa debe cerciorarse que tiene instalado rust y su gestor de paquetes.
Para esto basta con correr el comando :
              
              sudo apt get cargo  
       
 Luego de esto ejecute 
              
              cargo build
           
 Y finalmente puede proceder a correr el programa. 
 Este programa tiene dos modos para ejecutarse que son: Crear Backup y Restaurar de Backup. 
 
 
 Para crear un backup ejecute:
 
               cargo run mb {/input-dir/} {/backup-dir/} {pass} --release
              
  Para restaurar un backup ejecute
              
            cargo run rb {/backup-dir/} {/output-dir/} {pass} --release
            
  Recuerde que todos los directorios se deben pasar como rutas absolutas.
  
 En el directorio raiz del proyecto hay dos archivos bash ('run-backup.sh' y 'run-restore.sh'), puede basarse en ellos como ejemplo para
 ejecutar el programa.
 
