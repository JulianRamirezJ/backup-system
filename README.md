# Backup System
## Por: Julian Andres Ramirez Jimenez

Para ejecutar el programa debe cerciorarse que tiene instalado rust y su gestor de paquetes.
Para esto basta con correre el comando :
              
              sudo apt get cargo  
       
 Luego de esto ejecute 
              
              cargo build
           
 Y finalmente puede proceder a correr el programa. 
 Este programa tiene dos modospara ejecutarse que son: Crear Backup y Restaurar de Backup. 
 
 
 Para crear un backup ejecute:
 
               cargo run mb {/input-dir/} {/backup-dir/} {pass} --release
              
  Para restaurar un backup ejecute
              
            cargo run rb {/backup-dir/} {/output-dir/} {pass} --release
            
  Recuerde que todos los directorios se deben pasar como rutas absolutas.
 
