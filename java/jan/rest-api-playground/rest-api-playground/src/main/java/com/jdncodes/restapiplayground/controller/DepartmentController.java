package com.jdncodes.restapiplayground.controller;

import com.jdncodes.restapiplayground.entity.Department;
import com.jdncodes.restapiplayground.repository.DepartmentRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
//required args are final fields
@RequiredArgsConstructor
public class DepartmentController {

    private final DepartmentRepository departmentRepository;

    @PostMapping("/departments")
    public Department saveDepartment(@RequestBody Department department) {
        departmentRepository.save(department);
        return department;


    }
}
