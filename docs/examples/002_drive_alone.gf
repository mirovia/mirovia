###########
# Imports #
###########
com.mirovia.types.date
com.mirovia.functions.today


#########
# Types #
#########
date_of_birth
  format
   date
  constraints
   before_or_equal_today
    assert
      self <= today()
    error_message
      en
       Date must be inferior or equal to today


################
# Enumerations #
################
species
  dog
   english
    Dog
   french
    Chien
  human
   english
    Human
   french
    humain


country
  France
  USA


##########
# Inputs #
##########
date_of_birth
  title
   english
    Date of birth
   french
    Date de naissance
  type
   date_of_birth


country
  title
   english
    Country
   french
    Pays
  type
   country


species
  title
   english
    Species
   french
    Espèces
  type
   species


###########
# Outputs #
###########
age
  title
   english
    Age
   french
    Age
  type
   number


drive_alone_without_restriction
  title
   english
    Can drive alone without restriction
   french
    Peut conduire seul sans restriction
  type
   yes_no_maybe


explanation
  title
   english
    Explanation
   french
    Explication
  type
   text


#############
# Functions #
#############
compute_age
  inputs
   date_of_birth
  outputs
   age
  if
   date_of_birth <= today -> today - date_of_birth


can_drive_alone_without_restriction
  inputs
   age
   country
  outputs
   yes_no_maybe
   optional text
  if
   country == France    and 18 <= age    -> yes
   country == France    and     age < 18 -> no
   country == USA      and 18 <= age    -> yes
   country == USA      and 17 <= age < 18 -> maybe "Depends of the state."
                                    "Source: https://en.wikipedia.org/wiki/List_of_minimum_driving_ages#North_America."
   country == USA      and     age < 17 -> no


#########
# Graph #
#########
date_of_birth -> function.compute_age           -> age
age country  -> can_drive_alone_without_restriction -> drive_alone_without_restriction explanation
