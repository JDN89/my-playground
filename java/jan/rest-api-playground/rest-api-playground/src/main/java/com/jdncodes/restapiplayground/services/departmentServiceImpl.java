package com.jdncodes.restapiplayground.services;

import org.springframework.stereotype.Service;

@Service
public class departmentServiceImpl implements departmentService {
    @Override
    public String helloFromService() {
        return "Hello From Service";
    }
}
