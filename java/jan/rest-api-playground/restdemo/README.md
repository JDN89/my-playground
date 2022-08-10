##Lookup
- difference Iterable and List
  - pagingAndSortingRepository returns Iterable? look to documentation and examples
  - look into pagination documentation
  - look into what the sort parameter does and which string values can you pass it?


## TODO

- how to create and throw custom errors!
- unit tests


## Dummy DATA
 - Inject data via CommandLineRunner
  - Had to remove lombok from my models and Create getters,setters, ctors by hand -> look into it
 - connect to h2 db via: jdbc:h2:mem:people
  - name db "people" is specified in the yml file
 - use Faker library to create extra dummy data

#### Faker
- in dev , fake data with the faker library
- https://www.youtube.com/watch?v=UzBOv_SHUng

## Spring Data Pagination
- instead of returning all the data, only return info on a certain page/ a defined amount of records
- https://www.youtube.com/watch?v=oq-c3D67WqM

## Storing Secrets

- create a secrets.properties file
- add file to gitignore
- create a SecretConfigRecord
  - @ConfigurationProperties("xxx")
- import secrets.properties into your main properties or yml file
- EnableConfigurationProperties into  your main class 

## RECORDS - when to use in Spring boot?
link: https://www.youtube.com/watch?v=3NshiQIy7p4
- Data class
- spring boot configuration properties 
- Spring Data

#### info:
- records are immutable!
- You can add checks and logic through creation of a constructor. 
- You can't use records with Spring Data JPA, but you can use records wit JDBC




#### transfer DTO to entity and vice versa

#### create multidoc properties in yml file
- define different environments and set one environment to active

#### Yaml instead of properties.

#### Denpendency Injection
- Constructor based
- @autowirded
 
 > Senior Dev : Why are you using field injection instead of constructor injection?Junior Dev : What is field injection? I am using @Autowired

 > Field Injection uses reflection to set the values of private variables

 > Constructor Injection happens at the time of creating the object itself

  [article](https://eng.zemosolabs.com/when-not-to-autowire-in-spring-spring-boot-93e6a01cb793)

---

  ```Suggestion: With Lombok's @RequiredArgsConstructor, I can have all the advantages of Constructor Injection without having to manually adapt constructors when adding/changing/removing injected fields.... assuming I don't need to do anything fancy in the constructor, and the Lombok-generated one is sufficient.

Like this:
@RestController
@RequiredArgsConstructor
public class UserController {
private final UserService userService;
}
  ```


#### JPA and hibernate
#### Validation
#### Exception handling
- don't send whole stack traces to the front end (in case of an error)
#### logging
  - check impact logging on rest api
#### Unit tests
  - look into following statements(ctor injection vs @Autowired -> impact on unit testing):
    - no need to create a test-specific configuration component – dependencies are injected explicitly in a constructor
    - reclaimed freedom of using final keywords 
    - simple unit tests – reduced Spring Framework's overhead

#### integrate web socket?
#### Is their and async await equivalent in spring boot?
  - how do you make rest api asynchronous (or implemented behind the scenes)

#### Test performance - look for equivalent of Nbomber (dotnet)
- check impact logging on rest api


