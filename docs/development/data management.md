## Prelude

> For our small expected small data size, using database server seems
to be overhead. Rust iterator and its map capabilities should be enough
to effectively manage our data.

What can we gain with this?

Less resource need, same or higher efficiency, same 

## Core concept

Managing data in pure memory, and storing data in yaml files.

For this, we use a concept called "storage", storing data in yaml
files. Each data object has its own file, store it once^2 its changed,
otherwise loading once into memory and use it there.

Each storage has some kind of initialisation that creates storage
and loads data into it.

Storage has its own traits to manage its data.
**This part is under design.**

Example folder/file structure:

    data/  
        resource_A/  
            - data_object_1.yaml  
            - data_object_2.yaml  
            - data_object_3.yaml  
            ..  
            - data_object_N.yaml  
        resource_B/  
        resource_C/  
        ..  
        resource_N/  