package com.example.demo.controllers;

import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
//@RestController
//@ResponseBody
public class GreetingController {
    @RequestMapping("/")
    public String GetGreeting() {
        return "Dag Jan";
    }

}
