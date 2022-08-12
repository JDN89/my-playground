package com.jdn.restdemo.controller;

import com.jdn.restdemo.model.Address;
import com.jdn.restdemo.repository.AddressRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RequiredArgsConstructor
@RestController
@RequestMapping("api/v1/address")
public class AddressController {


    private final AddressRepository repository;

    // See if relation person and address is uniDirectional -> return only the addresses
    @GetMapping()
    public Iterable<Address> getAllAddresses () {
        return repository.findAll();
    }

}
