package com.jdncodes.restapiplayground.controller;

import com.jdncodes.restapiplayground.services.DepartmentService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequiredArgsConstructor
public class TestController {
    private final DepartmentService departmentService;
    @GetMapping("/hello")
    public String hello() {
        departmentService.helloFromService();
        return "Hello Jan_dn";
    }
}
