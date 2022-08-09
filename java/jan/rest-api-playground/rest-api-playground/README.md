## Dummy DATA
how to inject dummy test data into db? Commandlinerunner?
What is use of record? How to use it in Spring Boot?
## TODO

- Java 17, when is it better to replace your entity objects (classes) with records?
- DTO objects will be records, do we need class for our db setup?

#### transfer DTO to entity and vice versa

#### create multidoc properties in yml file
- define different environments and set one environment to active

#### inject dummy data in DB (if no data) upon starting the api

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

