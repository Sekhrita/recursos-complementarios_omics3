# recursos-complementarios_omics3

## Descarga de los Datos

Para comenzar con el análisis, es necesario descargar los datos desde el [GDC Data Portal](https://portal.gdc.cancer.gov). Sigue los siguientes pasos para obtener los datos `.maf` a partir del Cohort:

### Obtención y uso del Cohort

1. **Subir el Cohort:**
   - Ingresa a la página del [GDC Data Portal](https://portal.gdc.cancer.gov) y selecciona la opción **Cohort Builder**.
  
   - Importa el archivo del directorio de `cohort_base` denominado `cohort_12_PrimaryTypeCancer.2024-11-13.tsv`.

2. **Aplicar Filtros en la Sección Repository:**
   - **Data Category:** Simple Nucleotide Variation

   - **Data Format:** MAF
  
   - **Access:** Open
    
3. **Configuración de los Filtros (Opcional):**
   - El cohort de `cohort_base` se obtuvo mediante los siguientes filtros:
      - **Programs:** TCGA
   
      - **Projects:** Se seleccionan todos los disponibles.
    
      - **Primary Site:**
         - Bladder
           
         - Brain
         
         - Breast
         
         - Bronchus and lung
         
         - Colon
         
         - Kidney
         
         - Liver and intrahepatic bile duct
         
         - Ovary
         
         - Pancreas
         
         - Prostate Gland
         
         - Skin
         
         - Stomach
   
   - Esto resultó en aproximadamente **7077 datos**.

4. **Descarga del Manifest**
   - Una vez aplicados los filtros, descarga el Manifest de los datos resultantes. Este archivo permite reconocer y descargar los datos mediante su UUID.
  
   - Se recomienda utilizar la línea de comandos para descargar grandes volúmenes de datos.

### Descargar e Instalar el Cliente de GDC

Herramienta de línea de comandos que permite descargar y enviar datos de GDC. Recomendada para usuarios que requieran grandes transferencias de datos de GDC o necesiten descargar un gran número de archivos de datos. Se puede obtener y utilizar de la siguiente manera (información y documentación obtenida del [GDC Data Transfer Tool](https://gdc.cancer.gov/access-data/gdc-data-transfer-tool)):

1. **Descargar el Cliente de GDC** 
   ```bash
   wget https://gdc.cancer.gov/system/files/public/file/gdc-client_2.3_Ubuntu_x64-py3.8-ubuntu-20.04.zip
   ```

2. **Descomprimir el Archivo** 
   ```bash
   unzip gdc-client_2.3_Ubuntu_x64-py3.8-ubuntu-20.04.zip
   unzip gdc-client_2.3_Ubuntu_x64.zip
   ```

3. **Hacer el Cliente Ejecutable** 
   ```bash
   chmod +x gdc-client
   cp gdc-client /usr/local/bin/gdc-client
   ```

4. **Comprobar Instalación (Opcional)**
   ```bash
   gdc-client --version
   ```
   
5. **Limpieza de Directorio (Opcional)**
   ```bash
   rm -r gdc-client_2.3_Ubuntu_x64-py3.8-ubuntu-20.04.zip
   rm -r gdc-client_2.3_Ubuntu_x64.zip
   rm -r gdc-client
   ```
   
6. **Descargar los Datos** 
   ```bash
   gdc-client download -m /home/sekhrita/la_carpeta/omics/u3/manifest.txt -d /home/sekhrita/la_carpeta/omics/u3/data/ -n 8
   ```

## Procesamiento Paralelizado de Archivos MAF a CSV con Rust

### Instrucciones de uso

#### Instalación de Rust

Para correr este proyecto, primero necesitas instalar Rust en tu PC. Sigue los siguientes pasos:
 
1. **Instalar Rust:** Ejecuta este comando en tu terminal para instalar Rust usando `rustup`:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Agregar Rust al PATH:** Añade Rust al PATH para poder usarlo desde la terminal (normalmente esto se hace automáticamente después de instalar Rust):
   ```bash
   source $HOME/.cargo/env
   ```

3. **Verificar la Instalación:** Comprueba que Rust se haya instalado correctamente:
   ```bash
   rustc --version
   ```

#### Clonar el Proyecto y Compilarlo

1. **Clonar el Repositorio:** Clona este repositorio desde GitHub o descarga el código en tu máquina local.
   ```bash
   git clone <URL_DEL_REPOSITORIO>
   cd trial_rust-rayon
   ```

2. Instalar Dependencias: Todas las dependencias están definidas en el archivo `Cargo.toml`. Para descargar e instalar las dependencias necesarias, simplemente ejecuta:
   ```bash
   cargo build
   ```

   Esto instalará todas las bibliotecas necesarias, incluyendo `rayon` para paralelización y `flate2` para descomprimir los archivos `.gz`.

#### Cómo Correr el Script

1. **Ajustar el Número de Hilos:** Puedes configurar el número de hilos que el programa usará. Por defecto está configurado para usar 6 hilos. Para cambiar esto, edita la variable `num_threads` en `src/main.rs`:
   ```bash
   let num_threads = 8; // Ajusta este valor según la cantidad de hilos que deseas utilizar
   ```

2. **Ejecutar el Script:** Una vez hayas configurado el número de hilos, puedes correr el script con el siguiente comando:
   ```bash
   cargo run
   ```

   Este comando compilará y ejecutará el programa, que se encargará de procesar los archivos `.maf.gz` dentro de la carpeta `u3/data` y generar un archivo CSV combinado con la información filtrada.

### Funcionamiento del Script

- **Estructura de Carpetas:**
   - El script asume que existe una carpeta llamada `u3` en el mismo nivel que la carpeta del proyecto `trial_rust-rayon`.

   - Dentro de `u3`, debe haber una subcarpeta `data` que contenga carpetas con nombres UUID. Cada una de estas carpetas UUID contiene archivos `.maf.gz`.

- **Procesamiento en Lotes:**
   - Los archivos `.maf.gz` se procesan en lotes para evitar un uso excesivo de memoria. Cada lote se procesa en paralelo utilizando múltiples hilos.

   - Una vez procesado un archivo `.maf.gz`, su contenido se filtra y se agrega al archivo CSV final (`filtered_maf_combined.csv`).

   - Los archivos temporales se eliminan inmediatamente después de ser procesados para ahorrar espacio en disco.

### Personalización

- **Número de Hilos:** Puedes modificar el valor de `num_threads` en `src/main.rs` para ajustar el número de hilos que el script utilizará, dependiendo de la cantidad de núcleos disponibles en tu CPU.

- **Tamaño del Lote:** El tamaño del lote está definido por la variable `batch_size`, que por defecto está configurado en `100`. Puedes ajustar este valor para cambiar cuántos archivos se procesan en cada lote:
   ```bash
   let batch_size = 100;
   ```

   Aumentar este valor puede acelerar el procesamiento, pero también aumentará el uso de memoria.

- **Modificar Columnas Filtradas:** Las columnas filtradas del archivo `.maf` están definidas en la estructura `MafRecord` dentro de `src/main.rs`. Puedes modificar los campos en esta estructura para cambiar los datos que serán escritos en el archivo CSV final.

### Notas Finales

- **Recomendación de Espacio en Disco:** El procesamiento de los archivos `.maf.gz` puede requerir una cantidad considerable de espacio en disco debido a la descompresión y generación de archivos temporales; esto dependiendo de la cantidad definida de lotes. Asegúrate de tener al menos el doble del espacio requerido por los archivos originales (`u3/data` ocupa ~11Gb) disponible antes de ejecutar el script (se recomienda tener ~22Gb).

- **Condición de Carrera:** Para evitar conflictos al escribir en el archivo CSV combinado, se utiliza un mecanismo de bloqueo (`Mutex`). Esto asegura que solo un hilo pueda escribir en el archivo CSV a la vez, evitando problemas de condición de carrera (perdida de datos por conflicto entre dos procesos). 
