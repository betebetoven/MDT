# MDT - Transcriptor Médico para Record Operatorio

MDT es una aplicación innovadora escrita en Rust, diseñada para extraer información médica específica de audios. Utiliza las APIs de Whisper ASR y GPT-4 de OpenAI para transcribir y procesar información de audio médico en tiempo real.

## Características

- **Transcripción de Audio:** MDT utiliza la API de Whisper, un sistema de reconocimiento automático del habla (ASR, por sus siglas en inglés) de OpenAI, para transcribir los audios.

- **Procesamiento de Información:** MDT utiliza la API de GPT-4, también de OpenAI, para extraer y procesar información médica específica de los textos transcritos.

- **Manejo de Audio:** MDT utiliza ffmpeg para recortar y manejar los audios, facilitando la transcripción y el procesamiento de la información.

La combinación de estas características permite a MDT extraer y procesar eficazmente información médica de audios en formatos específicos.

## Ejemplo de Funcionalidad

A continuación, se presenta un ejemplo de cómo MDT puede extraer y procesar la información de un audio médico. Este texto fue obtenido de una cita médica de 30 minutos:

> **Nombre del Paciente:** Maravitulia Rodríguez
> 
> **Fecha de Consulta:** No especificada
>
> **Fecha de Nacimiento:** No especificada (edad de 21 años)
> 
> **Lugar de Nacimiento:** Cali
>
> **Dirección:** Vive en el barrio Atanasio Gerardo, Cali
>
> **Ocupación:** Impulsadora en un supermercado
>
> **Estado Civil:** En relación estable pero no convive con su pareja
>
> **Antecedentes Médicos:** Varicela durante su infancia, apendicitis durante la primaria, no toma medicamentos constantemente, hipertensión en la madre y diabetes en el padre, no alergias conocidas, vacunación completa durante la infancia
>
> **Motivo de Consulta:** Dolor al orinar que lleva alrededor de 4 o 5 días, con una sensación persistente de ardor, principalmente al final de la micción. No presenta cambios en el color o olor de la orina, ni presencia de abdominalgia, fiebre, vómitos. Adicionalmente, hace unos días antes de estos síntomas, empezó a notar flujo vaginal de color blanco y consistencia grumosa (similar a leche cortada), acompañado de picazón. Ha tenido episodios similares en los últimos meses. Asimismo, reporta irregularidades en su ciclo menstrual, usualmente con periodos de cada dos meses.
>
> **Examen Físico:** Se observa inflamación en la vulva y presencia del flujo descrito.
>
> **Impresión Clínica:** Posible candidiasis vaginal acompañada de dolor al orinar. También se sospecha la posible presencia de ovarios poliquísticos. 
>
> **Plan:** Se indicarán exámenes de orina y ecografía para confirmar el diagnóstico y estudiar las condiciones ováricas. Se iniciará tratamiento con medicamentos antifúngicos y manejo local con baños de asiento vinagre y agua. Se solicita que se realice una cita de seguimiento una vez que se obtengan los resultados de los exámenes.

Como se puede observar, MDT es capaz de transcribir y extraer la información médica específica de manera efectiva, transformando una cita médica en texto estructurado y fácilmente accesible.
