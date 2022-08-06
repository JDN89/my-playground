package com.jdncodes.restapiplayground.services;

import org.springframework.stereotype.Service;

@Service
public class DepartmentServiceImpl implements DepartmentService {
    @Override
    public Void helloFromService() {
        System.out.println("Hello from Service");
    return null;
    }
}
