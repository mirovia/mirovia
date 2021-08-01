import
   std.display.hex_game
display
   plateau
      type
         hex_game
      title
         english
            Greeting
         french
            Salutation
function
   hello
      output
         english
            Hello World!
         french
            Bonjour tout le monde !
graph
   function.hello -> display.salutation
