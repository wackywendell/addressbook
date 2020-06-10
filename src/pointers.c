#include <stdio.h>
#include <string.h>
#include <stdlib.h>

struct Contact
{
    char name[20];
    char address[20];
};

struct AddressBook
{
    int size;
    int capacity;
    struct Contact *contacts;
};

void add(struct AddressBook *book, struct Contact contact)
{
    if (book->size == book->capacity)
    {
        // Need to resize the array
        int new_size = book->size * 2;
        if (new_size == 0)
        {
            // Start with an array of size 2
            new_size = 2;
        }

        // To resize the array, we need to allocate a bigger one...
        struct Contact *new_array = (struct Contact *)malloc(new_size * sizeof(struct Contact));
        // copy the old one into the new one...
        memcpy(new_array, book->contacts, book->size * sizeof(struct Contact));
        // And free the old one
        free(book->contacts);
        // Set the pointer in book->contacts to point to the new_array
        book->contacts = new_array;
    }

    book->contacts[book->size] = contact;
    book->size++;
}

struct AddressBook create_book()
{
    // Create an AddressBook and two contacts on the stack...
    struct AddressBook book;
    struct Contact alice;
    struct Contact bob;
    struct Contact carol;

    book.contacts = NULL;
    book.size = 0;
    book.capacity = 0;

    strcpy(alice.name, "Alice");
    strcpy(alice.address, "123 Apple Ave");
    // Add this contact to the book
    // Doing so will allocate an array on the heap, and copy this contact into it,
    // copying it from the stack to the heap
    add(&book, alice);

    // Add another contact to the book
    strcpy(bob.name, "Bob");
    strcpy(bob.address, "123 Berry Boardwalk");
    // Copy this contact into the array of contacts
    add(&book, bob);

    // Add a third contact to the book
    strcpy(carol.name, "Carol");
    strcpy(carol.address, "123 Cherry Crossing");
    // Copy this contact into the array of contacts This is the third copy, so
    // it will trigger a reallocation, with a new array created, the alice and
    // bob copied into the new array, carl added after that, and the old array
    // freed
    add(&book, carol);

    return book;

    // When the function ends, the stack copies of alice, bob, and carol are
    // dropped, but the copies in the array on the heap live on
}

int main()
{
    struct AddressBook book = create_book();

    for (int i = 0; i < book.size; i++)
    {
        printf("%2d: %10s - %20s\n", i, book.contacts[i].name, book.contacts[i].address);
    }

    // When the function ends, the book on the stack will be dropped, but the
    // array it points to on the heap would remain
    //
    // Well, when this main function ends, the heap will also be cleaned up...
    // but in a normal function, this would be a memory leak
    free(book.contacts);
}