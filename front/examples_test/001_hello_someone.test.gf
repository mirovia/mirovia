display_input
   name
      title
         english
            Name
         french
            Nom
display
   personal_salutation
      title
         english
            Greeting
         french
            Salutation
function
   hello_someone
      input
         name
      output
         english
            Hello {name}!
         french
            Bonjour {name} !
graph
   input.name -> function.hello_someone
   function.hello_someone -> display.personal_salutation
