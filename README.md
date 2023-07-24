# MDT - Transcriptor Médico para Record Operatorio

MDT es una aplicación innovadora escrita en Rust, diseñada para extraer información médica específica de audios. Utiliza las APIs de Whisper ASR y GPT-4 de OpenAI para transcribir y procesar información de audio médico en tiempo real.

## Características

- **Transcripción de Audio:** MDT utiliza la API de Whisper, un sistema de reconocimiento automático del habla (ASR, por sus siglas en inglés) de OpenAI, para transcribir los audios.

- **Procesamiento de Información:** MDT utiliza la API de GPT-4, también de OpenAI, para extraer y procesar información médica específica de los textos transcritos.

- **Manejo de Audio:** MDT utiliza ffmpeg para recortar y manejar los audios, facilitando la transcripción y el procesamiento de la información.

La combinación de estas características permite a MDT extraer y procesar eficazmente información médica de audios en formatos específicos.
