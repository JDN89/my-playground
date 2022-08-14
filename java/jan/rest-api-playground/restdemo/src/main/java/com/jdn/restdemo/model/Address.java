package com.jdn.restdemo.model;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;

@Entity
public class Address {
    @Id
    @GeneratedValue
    private Integer id;
    private String address;
    private String city;

    public Address(String address, String city) {
        this.address = address;
        this.city = city;
    }

    public Address() {
    }

    // I generated also an all args constructor for the unit test person mocking
    // I needed an id for the itShouldSavePerson test -> creation person with id -> then findById
    public Address(Integer id, String address, String city) {
        this.id = id;
        this.address = address;
        this.city = city;
    }

    public Integer getId() {
        return id;
    }

    public void setId(Integer id) {
        this.id = id;
    }

    public String getAddress() {
        return address;
    }

    public void setAddress(String address) {
        this.address = address;
    }

    public String getCity() {
        return city;
    }

    public void setCity(String city) {
        this.city = city;
    }

    @Override
    public String toString() {
        return "Address{" + "id=" + id + ", address='" + address + '\'' + ", city='" + city + '\'' + '}';
    }
}